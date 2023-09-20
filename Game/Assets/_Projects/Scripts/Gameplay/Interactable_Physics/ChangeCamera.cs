using Cinemachine;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;

public class ChangeCamera : NetworkBehaviour
{
    [SerializeField] PlayerStateManager _stateManager;
    [SerializeField] CinemachineVirtualCamera _virtualCamera;
    [SerializeField] GameObject _player;
    [SerializeField] FPSCamera _fPSCameraPlayer;
    [SerializeField] private Transform _position;
    private Camera _playerCamera;

    [Networked(OnChanged = nameof(OnChangedOnUsing))]
    public NetworkBool IsUsing { get; set; }

    public static void OnChangedOnUsing(Changed<ChangeCamera> changed)
    {
        var b = changed.Behaviour;
        if (b.IsUsing)
        {
            b.Used();
        }
        else
        {
            b.Unused();
        }
    }

    void OnTriggerEnter(Collider other)
    {
        if(other.tag == "Player")
        {
            _fPSCameraPlayer = other.GetComponent<FPSCamera>();
            _player = other.gameObject;
        }
    }

    public void Toggle()
    {
        IsUsing = !IsUsing;
    }

    private void OnTriggerExit(Collider other)
    {
        if (other.tag == "Player")
        {
            _fPSCameraPlayer = null;
            _player = null;
        }
    }

    private void Start()
    {
        _playerCamera = Camera.main;
    }

    private void Spawned()
    {
        IsUsing = false;
    }

    void Used()
    {
        _stateManager.TriggerInteractState();
    }

    private void LateUpdate()
    {
        if (_stateManager.CurrentGameState == GameState.Interact)
        {
            _virtualCamera.gameObject.SetActive(true);
        } else if (_stateManager.CurrentGameState == GameState.Play)
        {
            _virtualCamera.gameObject.SetActive(false);
            if(_fPSCameraPlayer != null)
            _fPSCameraPlayer.SetCamera();
        }
    }

    void Unused()
    {
        _stateManager.TriggerPlayState();
    }

}
