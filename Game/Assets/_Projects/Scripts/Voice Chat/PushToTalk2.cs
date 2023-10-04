/*using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;
using Photon.Voice.Unity;
using UnityEngine.UI;

public class PushToTalk2 : MonoBehaviour
{
    private Recorder _recorder;
    public event InputActionReference pttKey;

    void Awake()
    {
        if (_recorder == null)
            _recorder = GetComponent<Recorder>();

        pttKey.action.performed += EnableTalking;
        pttKey.action.canceled += DisableTalking;
    }

    private void EnableTalking(InputAction.CallbackContext obj){
        _recorder.TransmitEnabled = true;
    }

    private void DisableTalking(InputAction.CallbackContext obj){
        _recorder.TransmitEnabled = false;
    }
}
*/