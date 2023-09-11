using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;

public class Bandul_Interact : MonoBehaviour
{
    [SerializeField] TextMeshProUGUI Ttext;
    public GameObject Bandul;
    public Meter meter;
    public HingeJoint hinge;
    public float value;
    public float Ctime;
    public bool Watch;
    public bool Busing;

    // Start is called before the first frame update
    void Start()
    {
        meter = Bandul.GetComponent<Meter>();
        Ctime = 0;
    }

    // Update is called once per frame
    void Update()
    {
        if (Watch == true) // stopwatch
        {
            Ctime = Ctime + Time.deltaTime;
        }
        Interact();
        Ttext.text = Ctime.ToString("f2");
    }
    void Interact()
    {
        if(Input.GetKeyDown("e"))
            {
                Watch = true;
            }
            else if (Input.GetKeyDown("q"))
            {
                Watch = false;
            }
            else if (Input.GetKeyDown("x"))
            {
                meter.reset();
                Ctime = 0f;
                Watch = false;
            }
    }
}
