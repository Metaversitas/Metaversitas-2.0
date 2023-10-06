using UnityEngine;
using TMPro;

public class SearchScript : MonoBehaviour
{
    public GameObject ContentHolder;

    public GameObject[] Element;

    public GameObject SearchBar;

    public int totalElements;
    private object searchBar;


    // Start is called before the first frame update
    void Start()
    {
        totalElements = ContentHolder.transform.childCount;    

        Element = new GameObject[totalElements];

        for (int i = 0; i < totalElements; i++)
        {
            Element[i] = ContentHolder.transform.GetChild(i).gameObject;    
        }
    }

    public void Search()
    {
        string SearchText = SearchBar.GetComponent<TMP_InputField>().text;
        int searchTxtlength = SearchText.Length;

        int searchedElements = 0;

        foreach(GameObject ele in Element)
        {
            searchedElements += 1;

            if (SearchText.ToLower() == ele.transform.GetChild(0).GetComponent<TextMeshProUGUI>().text.Substring(0, searchTxtlength).ToLower())
            {
                ele.SetActive(true);

            }
            else
            {
                ele.SetActive(false);
            }
        }
    }
}
