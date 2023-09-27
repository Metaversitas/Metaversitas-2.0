using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class InformasiObjek : MonoBehaviour
{
    public TextMeshProUGUI Keterangan; // Referensi ke elemen UI Text (TextMeshPro) untuk menampilkan keterangan
    private GameObject hoveredObject; // Objek yang sedang dihover oleh kursor

    private void Update()
    {
        // Raycast dari kursor mouse ke dunia 3D
        Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
        RaycastHit hit;

        // Cek apakah ada tumbukan dengan objek
        if (Physics.Raycast(ray, out hit))
        {
            // Ambil objek yang terkena raycast
            GameObject hitObject = hit.collider.gameObject;

            // Cek apakah objek yang terkena raycast berbeda dengan objek sebelumnya
            if (hitObject != hoveredObject)
            {
                // Update objek yang sedang dihover
                hoveredObject = hitObject;

                // Tampilkan nama objek di elemen UI Text (TextMeshPro)
                Keterangan.text = hoveredObject.name;

                // Aktifkan elemen UI Text (TextMeshPro) keterangan
                Keterangan.gameObject.SetActive(true);
            }
        }
        else
        {
            // Reset teks menjadi bandul ketika cursor tidak berada di atas objek
            Keterangan.text = "Bandul";

            // Reset objek yang sedang dihover
            hoveredObject = null;
        }
    }
}
