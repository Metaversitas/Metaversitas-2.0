using System.Collections;
using System.Collections.Generic;
using Unity.VisualScripting;
using UnityEngine;

public class Interactor : MonoBehaviour
{
    [SerializeField] private Meter _bandul;
    public virtual void OnMouseDown()
    {
        Debug.Log("OnMouseDown");
        _bandul.RPC_MouseClick();
        //_bandul.MousePosition();
    }

    public virtual void OnMouseUp()
    {
        Debug.Log("OnMouseUp");
        _bandul.RPC_MousePositionUP();
        //_bandul.MousePositionUP();
    }

    public virtual void OnMouseExit()
    {
        Debug.Log("OnMouseExit");
        _bandul.Rpc_StartCoroutine();
        //_bandul.StartCoroutine("ClearAngle");
    }
}
