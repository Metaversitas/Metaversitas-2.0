using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PauseMenu : MonoBehaviour
{
    public PlayerStateManager _playerStateManager;

    [SerializeField] private GameObject _pauseMenu;
    [SerializeField] private MenuManager _MenuManager;
    [SerializeField] private Menu _pauseNameMenu;
    bool _isOpen;

    // Start is called before the first frame update
    void Start()
    {
        _pauseMenu.SetActive(false);
    }

    // Update is called once per frame
    void Update()
    {
        if (_playerStateManager.CurrentGameState == GameState.Intro)
            return;

        if (_playerStateManager.CurrentGameState == GameState.Pause)
        {
            _pauseMenu.SetActive(true);
            _isOpen = true;
        }
        else if (Input.GetKeyDown("escape") && _playerStateManager.CurrentGameState != GameState.Interact)
        {
            _MenuManager.OpenMenu("Pause");
            if (_isOpen) 
            {
                _pauseNameMenu.Close();
                _isOpen = false;
            }
            //
        }
        else
        {
            _pauseMenu.SetActive(false);
            _isOpen= false;
        }
    }

    public void ResumeButton()
    {
        _playerStateManager.TriggerPlayState();
    }
}
