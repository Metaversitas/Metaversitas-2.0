using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI; // Impor namespace UI

public class BoundaryWarning : MonoBehaviour
{
    public GameObject warningCanvas;
    
    void Start()
    {
        // Nonaktifkan pesan peringatan pada awal permainan
        warningCanvas.SetActive(false);
    }

     private void OnTriggerEnter(Collider other)
    {
        if (other.CompareTag("Player"))
        {
            warningCanvas.SetActive(true);
        }
    }
     private void OnTriggerExit(Collider other)
    {
        if (other.CompareTag("Player"))
        {
            warningCanvas.SetActive(false);
        }
    }
}
