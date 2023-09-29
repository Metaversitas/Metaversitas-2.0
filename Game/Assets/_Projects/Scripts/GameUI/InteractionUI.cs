using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;

public class InteractionUI : NetworkBehaviour
{
    public GameObject _notifInformative;
    public GameObject _notifPraktikumMenu;
    [SerializeField] private PlayerStateManager _playerStateManager;

    // Start is called before the first frame update
    void Start()
    {
      _notifPraktikumMenu.SetActive(false);
      _notifInformative.SetActive(false);
    }

    private void OnTriggerEnter(Collider other)
    {
        if (Object.HasInputAuthority)
        {
            if (_playerStateManager.CurrentGameState == GameState.Play)
            {
                if (other.CompareTag("InformativeObject"))
                {
                    // Tambahkan notif tekan "E"
                    _notifInformative.SetActive(true);
                }
                if (other.CompareTag("OfflineMenu"))
                {
                    // Tambahkan notif tekan "B"
                    _notifPraktikumMenu.SetActive(true);
                }
                if (other.CompareTag("Alat_Praktikum"))
                {
                    // Tambahkan notif tekan "E"
                    _notifInformative.SetActive(true);
                }
            }
        }
        
    }

    private void OnTriggerStay(Collider other)
    {
        if (Object.HasInputAuthority)
        {
            if (_playerStateManager.CurrentGameState != GameState.Play)
            {
                _notifPraktikumMenu.SetActive(false);
                _notifInformative.SetActive(false);
            }
        } 
    }

    private void OnTriggerExit(Collider other)
    {
        if (Object.HasInputAuthority)
        {
            if (other.CompareTag("InformativeObject"))
            {
                _notifInformative.SetActive(false);
            }
            if (other.CompareTag("OfflineMenu"))
            {
                // Tambahkan notif tekan "B"
                _notifPraktikumMenu.SetActive(false);
            }
            if (other.CompareTag("Alat_Praktikum"))
            {
                // Tambahkan notif tekan "E"
                _notifInformative.SetActive(false);
            }
        }
        
    }
}
