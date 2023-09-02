using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Photon.Voice.Unity;

public class PushToTalk : MonoBehaviour
{
    private Recorder _recorder;
    // Start is called before the first frame update
    void Awake()
    {
        if (_recorder == null)
            _recorder = GetComponent<Recorder>();
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKey(KeyCode.V))
        {
            _recorder.TransmitEnabled = true;
        }
        else
        {
            _recorder.TransmitEnabled = false;
        }
    }
}
