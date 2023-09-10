using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class stateMenu : MonoBehaviour
{
    public PlayerStateManager gameStateManager;
    public GameObject pauseMenu;

    // Start is called before the first frame update
    void Start()
    {
        pauseMenu.SetActive(false);
        Debug.Log(gameStateManager.CurrentGameState);
    }

    // Update is called once per frame
    void Update()
    {
        if (gameStateManager.CurrentGameState == GameState.Pause)
        {
            Cursor.lockState = CursorLockMode.None;
            Cursor.visible = true;
            pauseMenu.SetActive(true);
        }
        else if (gameStateManager.CurrentGameState != GameState.Pause)
        {
            Cursor.lockState = CursorLockMode.Locked;
            Cursor.visible = false;
            pauseMenu.SetActive(false);
        }
    }
}
