using System.Collections;
using System.Collections.Generic;
using UnityEngine.UI;
using UnityEngine;
using System;
using Fusion;

public class ShowIconTalk : NetworkBehaviour
{
    [SerializeField] private Image imageToHideAndShow;
    [SerializeField] private Character _character;
    [Networked(OnChanged = nameof(OnChangedShowSpeaking))]
    public NetworkBool IsSpeaking { get; set; }

    public float delayInteract = 1f;

    [Networked]
    public Player Player { get; set; }

    [Networked]
    private TickTimer delay { get; set; }

    // Start is called before the first frame update

    public static void OnChangedShowSpeaking(Changed<ShowIconTalk> changed)
    {
        var b = changed.Behaviour;

        if (b.IsSpeaking)
        {
            b.imageToHideAndShow.enabled = true;
        }
        else
        {
            b.imageToHideAndShow.enabled = false;
        }
    }

    public override void Spawned()
    {
        delay = TickTimer.CreateFromSeconds(Runner, delayInteract);
        _character = GetComponent<Character>();

    }

    public void ToggleShow()
    {
        IsSpeaking = !IsSpeaking;
    }

    public override void FixedUpdateNetwork()
    {
        if (_character.Player == null) return;
        if (_character.Player.InputEnabled == false) return;

        if (GetInput(out InputData data))
        {
            _character._isReadInput = true;
            if (delay.ExpiredOrNotRunning(Runner) == false) return;

            if (data.GetButton(ButtonFlag.PTT))
            {
                delay = TickTimer.CreateFromSeconds(Runner, delayInteract);
                IsSpeaking = true;
                Debug.Log("Icon Active");
            }
            else // No input
            {
                IsSpeaking = false;
                Debug.Log("Icon Deactive");
            }
        }
    }
}
