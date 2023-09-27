using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class OfflineMenu : MonoBehaviour
{
    [SerializeField] GameObject _menu;
    [SerializeField] PlayerStateManager _playerStateManager;

    private void Start()
    {
        _menu.SetActive(false);
    }
    public void Open()
    {
        _menu.SetActive(true);
        _playerStateManager.TriggerInteractState();
    }

    public void Close()
    {
        _menu.SetActive(false);
        _playerStateManager.TriggerPlayState();
    }
}
