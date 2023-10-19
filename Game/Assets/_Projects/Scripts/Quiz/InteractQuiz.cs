using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;

public class InteractQuiz : MonoBehaviour
{
    [SerializeField] GameObject _menu;
    [SerializeField] PlayerStateManager _playerStateManager;

    private void Start()
    {
        _menu.SetActive(false);
    }

    // Update is called once per frame
    void Update()
    {
        if(Input.GetKeyUp(KeyCode.Q)) {
            if (_playerStateManager.CurrentGameState != GameState.Interact)
            {
                Open();
            }
        } else if (Input.GetKeyUp(KeyCode.Escape)) 
        {
            Close(); 
        }
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
