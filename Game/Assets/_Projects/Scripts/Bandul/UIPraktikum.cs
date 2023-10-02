using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;

public class UIPraktikum : NetworkBehaviour
{
    public GameObject kanvas;
    public bool IsActived;

    // Start is called before the first frame update
    void Start()
    {
        kanvas.SetActive(false);
    }

    public void Actived()
    {
            kanvas.SetActive(true);
            IsActived = true;
    }

    public void Deactived()
    {
            kanvas.SetActive(false);
            IsActived=false;
    }
}
