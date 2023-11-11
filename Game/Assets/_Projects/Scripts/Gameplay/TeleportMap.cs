using Fusion;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class TeleportMap : NetworkBehaviour
{

    public GameObject TP1;
    public GameObject TP2;

    Vector3 TP1Location;
    Vector3 TP2Location;

    public App app;
    public Character character;

    [SerializeField] public Button button1;
    [SerializeField] public Button button2;


    // Start is called before the first frame update
    void Start()
    {
        app = App.FindInstance();
        TP1Location = TP1.transform.position;
        TP2Location = TP2.transform.position;
        button1.onClick.AddListener(TeleportButton1Clicked);
        button2.onClick.AddListener(TeleportButton2Clicked);
    }

    // Update is called once per frame
    void Update()
    {
        if (character == null)
        {
            character = app.LocalCharacter;
        }
    }

    void TeleportButton1Clicked()
    {
        // Call the RPC_TeleportOurPlayer1 method when button1 is clicked
        character.RPC_TeleportOurPlayer(TP1Location);
    }
    void TeleportButton2Clicked()
    {
        // Call the RPC_TeleportOurPlayer1 method when button1 is clicked
        character.RPC_TeleportOurPlayer(TP2Location);
    }


    void TeleportOurPlayer(Vector3 tpLocation)
    {
        character.transform.position = tpLocation;
        Debug.Log(tpLocation + "Teleported");
    }
}
