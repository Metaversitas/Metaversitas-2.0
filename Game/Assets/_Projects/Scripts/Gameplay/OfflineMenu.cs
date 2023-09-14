using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class OfflineMenu : MonoBehaviour
{
    [SerializeField] GameObject _menu;
    [SerializeField] PlayerStateManager _stateManager;

    private void Start()
    {
        _menu.SetActive(false);
    }
    public void Open()
    {
        _menu.SetActive(true);
        _stateManager.TriggerInteractState();
    }

    public void Close()
    {
        _menu.SetActive(false);
        _stateManager.TriggerPlayState();
    }
}
