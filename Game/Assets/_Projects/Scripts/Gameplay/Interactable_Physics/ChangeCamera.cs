using Cinemachine;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;
using Photon.Realtime;

public class ChangeCamera : NetworkBehaviour
{
    [SerializeField] PlayerStateManager _playerStateManager;
    [SerializeField] CinemachineVirtualCamera _virtualCamera;
    [SerializeField] GameObject _player;
    [SerializeField] FPSCamera _fPSCameraPlayer;
    [SerializeField] private Transform _position;
    private Camera _playerCamera;
    [SerializeField] private UIPraktikum _playerUIPraktikum;
    CheckInteractionNetwork _checkInteractionNetwork;
    private void Spawned()
    {
        _playerCamera = Camera.main;
        _playerUIPraktikum = GetComponentInChildren<UIPraktikum>();
        _checkInteractionNetwork = GetComponent<CheckInteractionNetwork>();
    }

    private void Start()
    {
        _playerUIPraktikum = GetComponentInChildren<UIPraktikum>();
        _checkInteractionNetwork = GetComponent<CheckInteractionNetwork>();
    }

    void OnTriggerEnter(Collider other)
    {
        if(other.tag == "Player")
        {
            _fPSCameraPlayer = other.GetComponent<FPSCamera>();
            _player = other.gameObject;
        }
    }

    private void OnTriggerExit(Collider other)
    {
        if (other.tag == "Player")
        {
            _fPSCameraPlayer = null;
            _player = null;
        }
    }


    public void Used()
    {
        _playerStateManager.TriggerInteractState();
        _virtualCamera.gameObject.SetActive(true);
        _playerUIPraktikum.Actived();
        Rpc_Toogle();
    }

    [Rpc]
    void Rpc_Toogle()
    {
        _checkInteractionNetwork.Toggle();
    }


    public void Unused()
    {
        Rpc_Toogle();
        _playerStateManager.TriggerPlayState();
       _virtualCamera.gameObject.SetActive(false);
        if(_fPSCameraPlayer != null)
        {
            _fPSCameraPlayer.SetCamera();
        }
       _playerUIPraktikum.Deactived();
    }
}
