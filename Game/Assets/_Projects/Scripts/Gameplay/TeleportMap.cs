using Fusion;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class TeleportMap : NetworkBehaviour
{

    public GameObject TP1;
    public GameObject TP2;

    public GameObject Object;

    Vector3 TP1Location;
    Vector3 TP2Location;


    // Start is called before the first frame update
    void Start()
    {
        TP1Location = TP1.transform.position;
        TP2Location = TP2.transform.position;
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyDown("1"))
        {
            TeleportOurPlayer(TP1Location);
        }
        else if (Input.GetKeyDown("2"))
        {
            TeleportOurPlayer(TP2Location);
        }
    }

    void TeleportOurPlayer(Vector3 tpLocation)
    {
        Object.transform.position = tpLocation;
        Debug.Log(tpLocation + "Teleported");
    }
}
