using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Photon.Voice.Unity;
using System;

public class PushToTalk : MonoBehaviour
{
    private Recorder _recorder;
    public event EventHandler onTalk;
    public event EventHandler onNonTalk;

    [SerializeField] private GameObject settingVoice;
    // private _speakingIndicator _speakingIndicator;
    // Start is called before the first frame update
    void Awake()
    {
        if (_recorder == null)
            _recorder = GetComponent<Recorder>();
    }

    // Update is called once per frame
    void Update()
    {
        if (settingVoice.activeSelf == true)
        {
            ActivatePushToTalk();
        }
        else
        {
            DeactivatePushToTalk();
        }
        
    }

    public void ActivatePushToTalk()
    {
        if (Input.GetKey(KeyCode.V))
        {
            _recorder.TransmitEnabled = true;
            onTalk?.Invoke(this, EventArgs.Empty);
            // _speakingIndicator.gameObject.SetActive(true);
        }
        else
        {
            _recorder.TransmitEnabled = false;
            onNonTalk?.Invoke(this, EventArgs.Empty);
            // _speakingIndicator.gameObject.SetActive(false);
        }
    }

    public void DeactivatePushToTalk()
    {
        _recorder.TransmitEnabled = true;
    }
}
