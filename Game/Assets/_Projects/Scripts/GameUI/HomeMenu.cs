using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;
public class HomeMenu : MonoBehaviour
{
    // Start is called before the first frame update
    
    public void load_Scane()
    {
        SceneManager.LoadScene("MainMenu");
    }
    public void exit_Game()
    {
        Application.Quit();
        Debug.Log("Quit");
        UnityEditor.EditorApplication.isPlaying = false;
    }

}
