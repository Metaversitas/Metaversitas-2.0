using BrunoMikoski.AnimationsSequencer;
using Fusion;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InformativeObject : NetworkBehaviour
{
    public AnimationSequence showSeq;
    public AnimationSequence hideSeq;

    private GameObject localPlayerObject;
    public AudioSource audioSource;

    [Networked]
    public int CollidedPlayerCount { get; set; }

    [Networked(OnChanged = nameof(OnChangedOnShowing))]
    public NetworkBool IsShowing { get; set; }

    public static void OnChangedOnShowing(Changed<InformativeObject> changed)
    {
        var b = changed.Behaviour;

        if (b.IsShowing)
        {
            b.showSeq.Play();
            b.audioSource.Play();
        }
        else
        {
            b.audioSource.Stop();
            b.hideSeq.Play();
        }
    }

    public void ToggleShow()
    {
        IsShowing = !IsShowing;
    }

    public void ModifyCollidedCount(int value)
    {
        CollidedPlayerCount += value;

        // auto hide whenever no player is on interact area
        if (CollidedPlayerCount <= 0) IsShowing = false;
    }

    public void SetLocalPlayer(GameObject player)
    {
        localPlayerObject = player;
    }

    private void BillboardView()
    {
        var worldPos = transform.position + localPlayerObject.transform.rotation * Vector3.forward;
        //transform.LookAt(worldPos, localPlayerObject.transform.rotation * Vector3.up);
    }

    private void LateUpdate()
    {
        if (localPlayerObject == null) return;
        if (IsShowing == false) return;

        BillboardView();
    }
}
