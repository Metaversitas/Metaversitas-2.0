using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PauseMenu : MonoBehaviour
{
    public PlayerStateManager _playerStateManager;

    [SerializeField] private GameObject _pauseMenu;
    [SerializeField] private MenuManager _MenuManager;

    // Start is called before the first frame update
    void Start()
    {
        _pauseMenu.SetActive(false);
    }

    // Update is called once per frame
    void Update()
    {
        if (_playerStateManager.CurrentGameState == GameState.Pause)
        {
            _pauseMenu.SetActive(true);
            _MenuManager.OpenMenu("Pause");
        }
        else
        {
            _pauseMenu.SetActive(false);

        }
    }

    public void ResumeButton()
    {
        _playerStateManager.TriggerPlayState();
        _MenuManager.OpenMenu("Pause");
    }
}
