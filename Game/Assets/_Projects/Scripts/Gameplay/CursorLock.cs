using System.Collections;
using UnityEngine;

public class CursorLock : MonoBehaviour
{
    private bool _isCursorLocked = true;
    [SerializeField] private PlayerStateManager _gameStateManager;

    public bool IsLocked => _isCursorLocked;

    private void Start()
    {
        LockCursor();
    }

    private void Update()
    {
        if (Input.GetKeyDown(KeyCode.Escape))
            ToggleCursorLock();
        else if (_gameStateManager.CurrentGameState == GameState.Play)
            LockCursor();
        else if (_gameStateManager.CurrentGameState == GameState.Interact || _gameStateManager.CurrentGameState == GameState.Chatting)
            UnlockCursor();
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
        _gameStateManager.TriggerPlayState();
    }

    private void UnlockCursor()
    {
        Cursor.lockState = CursorLockMode.None;
        Cursor.visible = true;
        _isCursorLocked = false;
        if (_gameStateManager.CurrentGameState == GameState.Play)
        _gameStateManager.TriggerPauseState();
    }
}