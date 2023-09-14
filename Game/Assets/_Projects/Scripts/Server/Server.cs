// using UnityEngine;
// using UnityEngine.UI;
// using System.Collections;
// using UnityEngine.Networking;

// public class ServerAPI : MonoBehaviour
// {
//     [SerializeField] GameObject welcomePanel;
//     [SerializeField] Text user;
//     [Space]
//     [SerializeField] InputField username;
//     [SerializeField] InputField password;

//     [SerializeField] Text errorMessages;
//     [SerializeField] GameObject progressCircle;

//     [SerializeField] Button loginButton;

//     [SerializeField] string apiUrl; // URL API yang akan Anda panggil

//     UnityWebRequest www;

//     public void OnLoginButtonClicked()
//     {
//         loginButton.interactable = false;
//         progressCircle.SetActive(true);
//         StartCoroutine(Login());
//     }

//     IEnumerator Login()
//     {
//         // Buat URL lengkap dengan parameter yang sesuai
//         string url = apiUrl + "?username=" + username.text + "&password=" + password.text;

//         www = UnityWebRequest.Get(url);
//         yield return www.SendWebRequest();

//         if (www.result == UnityWebRequest.Result.ConnectionError || www.result == UnityWebRequest.Result.ProtocolError)
//         {
//             errorMessages.text = "Error: " + www.error;
//             Debug.Log("<color=red>" + www.downloadHandler.text + "</color>");
//         }
//         else
//         {
//             if (www.isDone)
//             {
//                 if (www.downloadHandler.text.Contains("error"))
//                 {
//                     errorMessages.text = "invalid username or password!";
//                     Debug.Log("<color=red>zz" + www.downloadHandler.text + "</color>");
//                 }
//                 else
//                 {
//                     //open welcome panel
//                     welcomePanel.SetActive(true);
//                     user.text = username.text;
//                     Debug.Log("<color=green>" + www.downloadHandler.text + "</color>");
//                 }
//             }
//         }

//         loginButton.interactable = true;
//         progressCircle.SetActive(false);

//         www.Dispose();
//     }
// }
