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

    public Player player;
    public Character character;

    [SerializeField] public Button button1;
    [SerializeField] public Button button2;


    // Start is called before the first frame update
    void Start()
    {
        player = FindAnyObjectByType<Player>();
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
            character = player.GetCharacter();
        }
    }

    void TeleportButton1Clicked()
    {
        // Call the RPC_TeleportOurPlayer1 method when button1 is clicked
        RPC_TeleportOurPlayer1();
    }
    void TeleportButton2Clicked()
    {
        // Call the RPC_TeleportOurPlayer1 method when button1 is clicked
        RPC_TeleportOurPlayer2();
    }

    [Rpc]
    void RPC_TeleportOurPlayer1()
    {
        TeleportOurPlayer(TP1Location);
    }
    [Rpc]
    void RPC_TeleportOurPlayer2()
    {
        TeleportOurPlayer(TP2Location);
    }


    void TeleportOurPlayer(Vector3 tpLocation)
    {
        character.transform.position = tpLocation;
        Debug.Log(tpLocation + "Teleported");
    }
}
