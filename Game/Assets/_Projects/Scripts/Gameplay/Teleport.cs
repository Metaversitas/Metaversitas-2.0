using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Teleport : MonoBehaviour
{
    [SerializeField] GameObject _menu;
    [SerializeField] PlayerStateManager _playerStateManager;
    // Start is called before the first frame update
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
    // Update is called once per frame
    void Update()
    {
        if (_playerStateManager.CurrentGameState == GameState.Intro)
            return;

        if (Input.GetKeyDown(KeyCode.M))
        {
            if (_playerStateManager.CurrentGameState == GameState.Play)
            {
                Open();
            }

        }
        if (Input.GetKeyDown(KeyCode.Escape))
        {
            Close();
        }
        Debug.Log(_playerStateManager.CurrentGameState);
    }
}
