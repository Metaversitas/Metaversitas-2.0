using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class npcInteract : MonoBehaviour
{
    public GameObject kanvas;

    // Start is called before the first frame update
    void Start()
    {
        kanvas.SetActive(false);
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    void OnTriggerEnter(Collider Collider)
    {
        if (Collider.gameObject.tag == "Player")
        {
            kanvas.SetActive(true);
        }
    }

    void OnTriggerExit(Collider Collider)
    {
        if (Collider.gameObject.tag == "Player")
        {
            kanvas.SetActive(false);
        }
    }
}
