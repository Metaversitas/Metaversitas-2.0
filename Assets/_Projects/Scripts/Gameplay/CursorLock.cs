using System.Collections;
using UnityEngine;

public class CursorLock : MonoBehaviour
{
    private bool _isCursorLocked = true;

    public bool IsLocked => _isCursorLocked;

    private void Start()
    {
        LockCursor();
    }

    private void Update()
    {
        if (Input.GetKeyDown(KeyCode.Escape))
            ToggleCursorLock();
    }

    private void OnApplicationFocus(bool isFocus)
    {
        if (isFocus) LockCursor();
    }

    public void ToggleCursorLock()
    {
        _isCursorLocked = !_isCursorLocked;
        if (_isCursorLocked)
            LockCursor();
        else
            UnlockCursor();
    }

    private void LockCursor()
    {
        _isCursorLocked = true;
        Cursor.lockState = CursorLockMode.Locked;
        Cursor.visible = false;
    }

    private void UnlockCursor()
    {
        Cursor.lockState = CursorLockMode.None;
        Cursor.visible = true;
        _isCursorLocked = false;
    }
}