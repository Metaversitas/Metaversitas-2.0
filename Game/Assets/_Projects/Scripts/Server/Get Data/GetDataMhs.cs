using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Networking;
using TMPro;
using System.Collections;

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

[System.Serializable]
public class Response
{
    public Data data;
    public bool status;
}

public class GetDataMhs : MonoBehaviour
{
    [SerializeField] private TextMeshProUGUI inGameNicknameText;
    [SerializeField] private TextMeshProUGUI nim;
    [SerializeField] private TextMeshProUGUI kodeKampus;
    [SerializeField] private TextMeshProUGUI programStudi;
    [SerializeField] private TextMeshProUGUI kodeProgram;
/*    [SerializeField] private TextMeshProUGUI facultyNameText;
    [SerializeField] private TextMeshProUGUI fullNameText;
    [SerializeField] private TextMeshProUGUI universityNameText;*/

    private static GetDataMhs instance;
    private Data userData;

    public static GetDataMhs Instance
    {
        get { return instance; }
    }

    private void Awake()
    {
        if (instance != null && instance != this)
        {
            Destroy(this.gameObject);
        }
        else
        {
            instance = this;
        }
    }

    void Start()
    {
        if (nim != null && kodeKampus != null && inGameNicknameText != null && kodeProgram != null)
        {
            GetData();
        }
        else
        {
            // Debug.LogError("One or more TextMeshProUGUI elements are not assigned in the Inspector.");
        }
    }


    public void GetData()
    {
        StartCoroutine(GetData_Coroutine());
    }

    IEnumerator GetData_Coroutine()
    {
        if (nim == null || kodeKampus == null || inGameNicknameText == null || kodeProgram == null)
        {
           // Debug.LogError("One or more TextMeshProUGUI elements are not assigned in the Inspector.");
            yield break;
        }

/*        facultyNameText.text = "Loading...";
        fullNameText.text = "Loading...";*/
        inGameNicknameText.text = "Loading...";
/*        universityNameText.text = "Loading...";*/

        string uri = "https://metaversitas.rweebz.xyz/user/profile";

        using (UnityWebRequest request = UnityWebRequest.Get(uri))
        {
            yield return request.SendWebRequest();

            if (request.result == UnityWebRequest.Result.ConnectionError || request.result == UnityWebRequest.Result.ProtocolError)
            {
                Debug.LogError("HTTP Error: " + request.error);
            }
            else
            {
                string json = request.downloadHandler.text;
                Response response = JsonUtility.FromJson<Response>(json);

                if (response != null && response.status)
                {
                    userData = response.data;

/*                    facultyNameText.text = $"{userData.faculty_name}";
                    fullNameText.text = $"{userData.full_name}";*/
                    inGameNicknameText.text = $"{userData.in_game_nickname}";
/*                    universityNameText.text = $"{userData.university_name}";*/

                    nim.text = $"{userData.user_university_id}";
                    kodeKampus.text = $"{userData.user_id}"; //sementara pake user id
                    programStudi.text = $"{userData.faculty_name}";
                    kodeProgram.text = $"{userData.faculty_id}";

                    Debug.Log("Received Data: " + json);
                }
                else
                {
                    Debug.LogError("Invalid data or status is false.");
                }
            }
        }
    }
}
