using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InteractableObject_Info : InteractableObject
{
    bool Open = false;
    public GameObject Ui;
    public override void Interact()
    {
       if(!Open)
        {
            Ui.SetActive(true);
            Open = true;
        }
        else if (Open)
        {
            Ui.SetActive(false);
            Open = false;
        }
    }
            
}
