using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using Fusion;

public class Bandul_Interact : NetworkBehaviour
{
    [SerializeField] TextMeshProUGUI Ttext;
    [SerializeField] private GameObject Bandul;
    private Meter meter;
    private UIPraktikum _uIPraktikum;
    [SerializeField] private HingeJoint hinge;
    [SerializeField] private float value;
    [SerializeField] private float Ctime;
    [SerializeField] private bool Watch;

    // Start is called before the first frame update
    void Start()
    {
        _uIPraktikum = GetComponent<UIPraktikum>();
        meter = Bandul.GetComponent<Meter>();
        Ctime = 0;
    }
    // Update is called once per frame
    void Update()
    {
        Interact();
        if (Watch == true) // stopwatch
        {
            Ctime = Ctime + Time.deltaTime;
        }
        Ttext.text = Ctime.ToString("f2");
    }
    void Interact()
    {
        if (_uIPraktikum.IsActived)
        {
            if (Input.GetKeyDown("e"))
            {
                Watch = true;
            }
            else if (Input.GetKeyDown("q"))
            {
                Watch = false;
            }
            else if (Input.GetKeyDown("x"))
            {
                meter.RPC_Reset();
                Ctime = 0f;
                Watch = false;
            }
        }
    }
}
