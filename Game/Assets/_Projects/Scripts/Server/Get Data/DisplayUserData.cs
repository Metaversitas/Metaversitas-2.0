using Metaversitas.User;
using UnityEngine;
using TMPro;

public class DisplayUserData : MonoBehaviour
{
    public App app;

    [SerializeField] private TMP_Text fullName;
    [SerializeField] private TMP_Text nim;
    [SerializeField] private TMP_Text facultyid;
    [SerializeField] private TMP_Text prodi;
    public string gender;
    public string role;

    void Start()
    {
        app = App.FindInstance();

        // Get the user data.
        UserData userData = app.get_userManager().get_userData();

        // Convert the user data to TMPro.TMP_Text objects.
        fullName.SetText(userData.get_fullName());
        nim.SetText(userData.get_userUniversityID().ToString());
        facultyid.SetText(userData.get_facultyID().ToString());
        prodi.SetText(userData.get_facultyName());
    }
}
