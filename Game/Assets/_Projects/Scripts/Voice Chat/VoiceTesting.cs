// using UnityEngine;
// using UnityEngine.UI;

// public class ImageControl : MonoBehaviour
// {
//     public TalkEventHandler talkEventHandler; // Menghubungkan skrip ini dengan skrip TalkEventHandler
//     public Image imageToControl; // Objek Image yang ingin Anda kontrol

//     private void Start()
//     {
//         // Menambahkan listener ke event 'onTalk' dari TalkEventHandler
//         talkEventHandler.onTalk.AddListener(ActivateImage);
        
//         // Menambahkan listener ke event 'onNonTalk' dari TalkEventHandler
//         talkEventHandler.onNonTalk.AddListener(DeactivateImage);
//     }

//     private void ActivateImage()
//     {
//         // Mengaktifkan objek Image saat event 'onTalk' terjadi
//         imageToControl.gameObject.SetActive(true);
//     }

//     private void DeactivateImage()
//     {
//         // Menonaktifkan objek Image saat event 'onNonTalk' terjadi
//         imageToControl.gameObject.SetActive(false);
//     }
// }

// using UnityEngine;
// using UnityEngine.Events;

// public class TalkEventHandler : MonoBehaviour
// {
//     public UnityEvent onTalk;
//     public UnityEvent onNonTalk;

//     private void Update()
//     {
//         // Mengecek apakah tombol 'V' ditekan
//         if (Input.GetKeyDown(KeyCode.V))
//         {
//             // Memanggil event 'Talk' jika tombol 'V' ditekan
//             onTalk.Invoke();
//         }
//         else
//         {
//             // Memanggil event 'NonTalk' jika tombol 'V' tidak ditekan
//             onNonTalk.Invoke();
//         }
//     }
// }