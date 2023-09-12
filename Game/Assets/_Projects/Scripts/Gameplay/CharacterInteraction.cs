using Fusion;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CharacterInteraction : NetworkBehaviour
{
    public float delayInteract = 1f;
    public GameObject _notifInformative;
    public GameObject _notifPraktikumMenu;

    [Networked]
    public InformativeObject CurrentObject { get; set; }

    [Networked] 
    private TickTimer delay { get; set; }

    [Networked] 
	public Player Player { get; set; }

    private void Start()
    {
        _notifInformative.SetActive(false);
    }
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
                CurrentObject.SetLocalPlayer(gameObject);
            // Tambahkan notif tekan "E"
            _notifInformative.SetActive(true);
        } else if (other.CompareTag("PraktikumMenu"))
        {
            // Tambahkan notif tekan "B"
            _notifPraktikumMenu.SetActive(true);
        }
    }

    private void OnTriggerExit(Collider other)
    {
        if (other.CompareTag("InformativeObject"))
        {
            if (CurrentObject == null) return;

            CurrentObject.ModifyCollidedCount(-1);
            CurrentObject = null;
            // Tambahkan notif tekan "E"
            _notifInformative.SetActive(false);
        }
        else if (other.CompareTag("PraktikumMenu"))
        {
            // Tambahkan notif tekan "B"
            _notifPraktikumMenu.SetActive(false);
        }

    }

    private void TryInteract()
    {
        if (CurrentObject == null) return;

        CurrentObject.ToggleShow();
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
                TryInteract();
            }
        }
	}
}
