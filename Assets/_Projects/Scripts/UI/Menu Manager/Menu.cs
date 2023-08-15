using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Metaversitas.Constants;

public class Menu : MonoBehaviour
{

  [SerializeField] public Enum_Menu menuName;
  public bool open;

  private void Awake()
  {
    //this.menu_manager = MenuManager.Instance;
  }

  public void Open()
    {
        open = true;
        gameObject.SetActive(true);
    }

    public void Close()
    {
        open = false;
        gameObject.SetActive(false);
    }
}