using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ChatShowHide : MonoBehaviour
{
    public PlayerStateManager _playerStateManager;
    public GameObject _chatMenu;
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        if (_playerStateManager.CurrentGameState == GameState.Pause|| _playerStateManager.CurrentGameState == GameState.Interact) {
            _chatMenu.SetActive(false);
        } else
            _chatMenu.SetActive(true);
    }
}
