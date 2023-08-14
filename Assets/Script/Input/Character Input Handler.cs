using System;
using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.InputSystem.Utilities;

public class CharacterInputHandler : MonoBehaviour
{
    public PlayerStateManager gameStateManager;
    public GameState CurrentGameState;
    //Variable
    Vector2 viewInputVector = Vector2.zero;
    Vector2 moveInputVector = Vector3.zero;
    bool isJumpButtonPressed = false;

    //Other Component
    PlayerInputAction playerInputActions;
    LocalCameraHandler localCameraHandler;
    PlayerInteractionHandler playerInteractionHandler;
    stateMenu pauseMenu;


    // Start is called before the first frame update

    private void Awake()
    {
        localCameraHandler = GetComponentInChildren<LocalCameraHandler>();
        playerInteractionHandler = GetComponent<PlayerInteractionHandler>();
        pauseMenu = GetComponentInChildren<stateMenu>();
        playerInputActions = new PlayerInputAction();

        playerInputActions.Player.Jump.started += _ => Jump();
        playerInputActions.Player.Interact.started += _ => Interact();
        playerInputActions.Player.Pause.started += _ => PauseState();
    }

    private void PauseState()
    {
        if (gameStateManager.CurrentGameState == GameState.Play)
        {
            gameStateManager.TriggerPauseState();
        }
        else
        {
            gameStateManager.TriggerPlayState();
        }
    }

    private void Interact()
    {
        playerInteractionHandler.Interact();
    }


        private void Start()
    {
        Cursor.lockState = CursorLockMode.Locked;
        Cursor.visible = false;
    }

    private void Jump()
    {
        isJumpButtonPressed = true;
        
    }

    void OnEnable()
    {
        playerInputActions.Enable();
    }

    private void OnDisable()
    {
        playerInputActions.Disable();
    }
    public Vector3 GetMovementVectorNormalized()
    {
        Vector3 inputVector = playerInputActions.Player.Move.ReadValue<Vector3>();
        inputVector = inputVector.normalized;
        return inputVector;
    }

    public Vector2 GetViewVectorNormalized()
    {
        Vector2 ViewinputVector = playerInputActions.Player.Look.ReadValue<Vector2>();
        ViewinputVector = ViewinputVector.normalized * 5;
        return ViewinputVector;
    }

    private void Update()
    {
        viewInputVector = GetViewVectorNormalized();
        viewInputVector.y = viewInputVector.y * -1;
        localCameraHandler.SetViewInputVector(viewInputVector);

        moveInputVector = GetMovementVectorNormalized();

    }

    public NetworkInputData GetNetworkInput()
    {
        NetworkInputData networkInputData = new NetworkInputData();
        // View data
        networkInputData.aimForwardVector = localCameraHandler.transform.forward;

        //Move data
        networkInputData.movementInput = moveInputVector;

        //Jump data
        networkInputData.isJumpPressed = isJumpButtonPressed;

        //Reset variables now that we have read their states
        isJumpButtonPressed = false;

        return networkInputData;
    }

}
