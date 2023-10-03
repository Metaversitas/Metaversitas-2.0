using System.Collections;
using System.Collections.Generic;
using UnityEngine.UI;
using UnityEngine;
using System;
using Fusion;

public class ShowIconTalk : NetworkBehaviour
{
    [SerializeField] private Image imageToHideAndShow;
    [Networked(OnChanged = nameof(OnChangedShowSpeaking))]
    public NetworkBool IsSpeaking { get; set; }

    // Start is called before the first frame update
    void Start()
    {
        PushToTalk pushToTalk = FindObjectOfType<PushToTalk>();
        pushToTalk.onTalk += PushTotalk_Active;
        pushToTalk.onNonTalk += PushTotalk_Deactive;
    }

    public static void OnChangedShowSpeaking(Changed<ShowIconTalk> changed)
    {
        var b = changed.Behaviour;

        if (b.IsSpeaking)
        {
            b.imageToHideAndShow.enabled = true;
            Debug.Log("Icon Active");
        }
        else
        {
            b.imageToHideAndShow.enabled = false;
            Debug.Log("Icon Not-Active");
        }
    }

    public void ToggleShow()
    {
        IsSpeaking = !IsSpeaking;
    }

    private void PushTotalk_Active(object sender, EventArgs e){
        IsSpeaking = true;
    }

    private void PushTotalk_Deactive(object sender, EventArgs e){
        IsSpeaking = false;
    }
}
