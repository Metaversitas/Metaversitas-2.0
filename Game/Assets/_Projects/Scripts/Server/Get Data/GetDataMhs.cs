using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Networking;
using TMPro;
using System.Collections;



// Kelas baru yang merepresentasikan data dari API
[System.Serializable]
public class Data
{
    public int faculty_id;
    public string faculty_name;
    public string full_name;
    public string in_game_nickname;
    public string university_name;
    public string user_id;
    public string user_univ_role;
    public int user_university_id;
}

// Kelas baru yang merepresentasikan respons dari API
[System.Serializable]
public class Response
{
    public Data data;
    public bool status;
}

public class GetDataMhs : MonoBehaviour
{
    [SerializeField]
    private TextMeshProUGUI outputArea; // Mengganti tipe data menjadi TextMeshProUGUI

    // Singleton instance
    private static GetDataMhs instance;

    public static GetDataMhs Instance
    {
        get { return instance; }
    }

    // Variabel baru untuk menyimpan data dari API
    private Data userData;

    private void Awake()
    {
        // Cek jika instance sudah ada
        if (instance != null && instance != this)
        {
            // Destroy this object jika instance sudah ada
            Destroy(this.gameObject);
        }
        else
        {
            // Set instance ke objek ini
            instance = this;
        }
    }

    void Start()
    {
        if (outputArea != null)
        {
            // Panggil GetData langsung saat Start
            GetData();
        }
        else
        {
            Debug.LogError("OutputArea is not assigned in the Inspector.");
        }
    }

    public void GetData()
    {
        StartCoroutine(GetData_Coroutine());
    }

    IEnumerator GetData_Coroutine()
    {
        if (outputArea == null)
        {
            Debug.LogError("OutputArea is not assigned in the Inspector.");
            yield break;
        }

        outputArea.text = "Loading...";
        string uri = "https://metaversitas.rweebz.xyz/user/profile";

        using (UnityWebRequest request = UnityWebRequest.Get(uri))
        {
            yield return request.SendWebRequest();

            if (request.result == UnityWebRequest.Result.ConnectionError || request.result == UnityWebRequest.Result.ProtocolError)
            {
                outputArea.text = request.error;
                Debug.LogError("HTTP Error: " + request.error);
            }
            else
            {
                // Parse JSON response
                string json = request.downloadHandler.text;
                Response response = JsonUtility.FromJson<Response>(json);

                if (response != null && response.status)
                {
                    // Simpan data ke variabel userData
                    userData = response.data;

                    // Akses nilai-nilai dari objek userData
                    string facultyName = userData.faculty_name;
                    string fullName = userData.full_name;
                    string inGameNickname = userData.in_game_nickname;
                    string universityName = userData.university_name;

                    // Tampilkan data di TextMeshProUGUI
                    outputArea.text = $"Faculty: {facultyName}\nFull Name: {fullName}\nIn-Game Nickname: {inGameNickname}\nUniversity: {universityName}";
                    Debug.Log("Received Data: " + json);
                }
                else
                {
                    outputArea.text = "Data not found or status is false.";
                    Debug.LogError("Invalid data or status is false.");
                }
            }
        }
    }
}
