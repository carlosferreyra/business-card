import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]


def load_script(name):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


sync = load_script("sync_resume")
release = load_script("release_pypi")
verify = load_script("verify_release")


class ScriptTests(unittest.TestCase):
    def test_embedded_snapshot_passes_sync_validation(self):
        sync.validate_resume((ROOT / "resume.json").read_text())

    def test_invalid_snapshot_does_not_overwrite_fallback(self):
        data = json.loads((ROOT / "resume.json").read_text())
        del data["profiles"]["business-card"]
        with tempfile.TemporaryDirectory() as directory:
            destination = Path(directory) / "resume.json"
            destination.write_text("original")
            with patch.object(sync, "DEST", destination), patch.object(sync.httpx, "get") as get:
                get.return_value.text = json.dumps(data)
                with self.assertRaises(KeyError):
                    sync.main()
            self.assertEqual(destination.read_text(), "original")

    def test_malformed_link_is_rejected(self):
        data = json.loads((ROOT / "resume.json").read_text())
        del data["links"][0]["url"]
        with self.assertRaises(KeyError):
            sync.validate_resume(json.dumps(data))

    def wrapper(self):
        meta = release.PackageMetadata.from_cargo(ROOT)
        namespace = {"__name__": "wrapper"}
        exec(release.Templates.CLI_WRAPPER.format(meta=meta), namespace)
        return namespace

    def test_python_windows_bootstrap_fails_without_download(self):
        wrapper = self.wrapper()
        with patch("platform.system", return_value="Windows"), patch("pathlib.Path.exists", return_value=False), patch("subprocess.run") as run:
            self.assertEqual(wrapper["main"](), 1)
            run.assert_not_called()

    def test_python_installer_failure_returns_nonzero(self):
        wrapper = self.wrapper()
        with patch("platform.system", return_value="Linux"), patch("pathlib.Path.exists", return_value=False), patch("subprocess.run", side_effect=subprocess.CalledProcessError(22, "curl")):
            self.assertEqual(wrapper["main"](), 1)

    def test_python_forwards_arguments_and_exit_code(self):
        wrapper = self.wrapper()
        with patch("pathlib.Path.exists", return_value=True), patch("sys.argv", ["wrapper", "--version"]), patch("subprocess.run") as run:
            run.return_value.returncode = 7
            self.assertEqual(wrapper["main"](), 7)
            self.assertEqual(run.call_args.args[0][1:], ["--version"])

    def test_release_guard_accepts_matching_commit(self):
        package = release.PackageMetadata.from_cargo(ROOT)
        record = {"draft": False, "assets": [{"name": f"{package.name}-installer.sh"}]}
        with patch.object(verify.subprocess, "check_output", side_effect=[json.dumps(record), "abc\n", "abc\n"]), patch.object(verify.subprocess, "run"):
            verify.main()

    def test_release_guard_rejects_wrong_commit(self):
        package = release.PackageMetadata.from_cargo(ROOT)
        record = {"draft": False, "assets": [{"name": f"{package.name}-installer.sh"}]}
        with patch.object(verify.subprocess, "check_output", side_effect=[json.dumps(record), "abc\n", "def\n"]), patch.object(verify.subprocess, "run"):
            with self.assertRaisesRegex(RuntimeError, "does not match"):
                verify.main()

    def test_release_guard_rejects_draft_or_missing_installer(self):
        for record in [{"draft": True}, {"draft": False, "assets": []}]:
            with self.subTest(record=record), patch.object(verify.subprocess, "check_output", return_value=json.dumps(record)), patch.object(verify.subprocess, "run") as run:
                with self.assertRaises(RuntimeError):
                    verify.main()
                run.assert_not_called()


if __name__ == "__main__":
    unittest.main()
