using Fusion;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CharacterInteraction : NetworkBehaviour
{
    public float delayInteract = 1f;
    [SerializeField] private PlayerStateManager _playerStateManager;

    [Networked]
    public InformativeObject CurrentObject { get; set; }
    [Networked]
    public CheckInteractionNetwork _alatPraktikumNetwork { get; set; }
    public ChangeCamera _alatPraktikumLocal;

    [Networked] 
    private TickTimer delay { get; set; }

    [Networked] 
	public Player Player { get; set; }

    public OfflineMenu _offlineMenu;
    public override void Spawned()
    {
        delay = TickTimer.CreateFromSeconds(Runner, delayInteract);
    }

    private void OnTriggerEnter(Collider other)
    {
        if (other.CompareTag("InformativeObject"))
        {
            var informativeObject = other.GetComponent<InformativeObject>();
            CurrentObject = informativeObject;
            CurrentObject.ModifyCollidedCount(1);

            if (Object.HasInputAuthority)
            {
                CurrentObject.SetLocalPlayer(gameObject);
            }
        } 
        if (other.CompareTag("OfflineMenu"))
        {
            var offlineMenu = other.GetComponent<OfflineMenu>();
            _offlineMenu = offlineMenu;
        }
        if (other.CompareTag("Alat_Praktikum"))
        {
            var AlatPraktikumLocal = other.GetComponent<ChangeCamera>();
            var AlatPraktikumNetwork = other.GetComponent<CheckInteractionNetwork>();
            _alatPraktikumLocal = AlatPraktikumLocal;
            _alatPraktikumNetwork = AlatPraktikumNetwork;
        }
    }

    private void OnTriggerExit(Collider other)
    {
        if (other.CompareTag("InformativeObject"))
        {
            if (CurrentObject == null) return;

            CurrentObject.ModifyCollidedCount(-1);
            CurrentObject = null;
        }
        if (other.CompareTag("OfflineMenu"))
        {
            if (_offlineMenu == null) return;
            _offlineMenu = null;
        }
        if (other.CompareTag("Alat_Praktikum"))
        {
            if (_alatPraktikumLocal == null) return;
            _alatPraktikumLocal = null;
            if (_alatPraktikumNetwork == null) return;
            _alatPraktikumNetwork = null;
        }
    }

    private void TryInteract()
    {
        if (CurrentObject != null)
        {
            CurrentObject.ToggleShow();
        }
        else if (_alatPraktikumLocal != null) 
        {
            Debug.Log("mencoba akses ke _alatpraktikum" + Object.HasInputAuthority);
            if(_alatPraktikumNetwork.IsUsing == false && Object.HasInputAuthority)
            {
                Debug.Log("Mengakses _alatpraktikum");
                _alatPraktikumLocal.Used();
                _playerStateManager.TriggerInteractState();
            } else if(_alatPraktikumNetwork.IsUsing == true && Object.HasInputAuthority)
            {
                Debug.Log("Lagi Dipake");
            }
        } else return;
    }


    private void TryInteractOffline()
    {
        if (_offlineMenu == null) return;
        _offlineMenu.Open();
        _playerStateManager.TriggerInteractState();
    }

    private void TryUninteract()
    {
        if (_alatPraktikumLocal != null)
        {
            if (_alatPraktikumNetwork.IsUsing == true && Object.HasInputAuthority)
            {
                _alatPraktikumLocal.Unused();
                _playerStateManager.TriggerPlayState();
            }
        }
    }

    private void Update()
    {
        if (Input.GetKeyDown("b")) { TryInteractOffline(); }
    }

    public override void FixedUpdateNetwork()
	{
		if (Player == null) return;
        if (Player.InputEnabled == false) return;

		if (GetInput(out InputData data))
		{
            if (delay.ExpiredOrNotRunning(Runner) == false) return;

            if (data.GetButton(ButtonFlag.INTERACT))
            {
                delay = TickTimer.CreateFromSeconds(Runner, delayInteract);
                if (_playerStateManager.CurrentGameState == GameState.Play)
                TryInteract();
            }
            if (data.GetButton(ButtonFlag.ESCAPE))
            {
                delay = TickTimer.CreateFromSeconds(Runner, delayInteract);
                TryUninteract();
            }
        }
	}
}
