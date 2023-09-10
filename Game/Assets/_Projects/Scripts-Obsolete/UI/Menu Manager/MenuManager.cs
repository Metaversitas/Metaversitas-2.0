using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEditor;
using Metaversitas.Constants;


/**
    <summary>
    The MenuManager class manages menus and their states within the application.
    </summary>
**/
public class MenuManager : MonoBehaviour
{
    public static MenuManager Instance;
    [SerializeField] Menu[] menus;
    [SerializeField] public Dictionary<Enum_Menu, Menu> menu_map = new Dictionary<Enum_Menu, Menu>();
    [SerializeField] public Stack<Enum_Menu> menu_history = new Stack<Enum_Menu>();

    private bool is_loading_menu = false;
    private Enum_Menu menu_to_load;

    private bool is_need_loading = false;
    public bool is_all_closed = false;


    /**
        <summary>
        Awake is called when the script instance is being loaded.
        It ensures that only one instance of MenuManager exists.
        </summary>
    **/
    private void Awake()
    {
        if (Instance == null)
        {
            Instance = this;
        }
        else if (Instance != this)
        {
            Destroy(this);
            return;
        }

        foreach (Menu menu in menus)
        {
            if (!menu_map.ContainsKey(menu.menuName))
            {
                //Debug.Log("Adding menuName: " + menu.menuName);
                menu_map.Add(menu.menuName, menu);
            }
        }
    }

    /**
        <summary>
        Opens a specific menu by its Enum_Menu identifier.
        </summary>
        <param name="menu">The Enum_Menu identifier of the menu to be opened.</param>
    **/
    public void OpenMenu(Enum_Menu menu)
    {
        if (this.is_loading_menu)
        {
            return;
        }

        this.menu_to_load = menu;
        this.is_all_closed = false;

        if (this.shouldShowLoadingScreen(menu))
        {
            this.is_loading_menu = true;

            if (this.menu_history.Count > 0)
            {
                Enum_Menu curr_menu = this.menu_history.Peek();
                this.CloseMenu(curr_menu);
            }

            Menu loading_menu = this.menu_map[Enum_Menu.Loading];
            loading_menu.Open();

            StartCoroutine(LoadMenuCoroutine());
        }
        else
        {
            if (this.menu_history.Count > 0)
            {
                Enum_Menu curr_menu = this.menu_history.Peek();
                this.CloseMenu(curr_menu);
            }

            this.menu_map[menu].Open();

            this.menu_history.Push(menu);
        }
    }

    /**
        <summary>
        Closes a specific menu by its Enum_Menu identifier.
        </summary>
        <param name="menu">The Enum_Menu identifier of the menu to be closed.</param>
    **/
    public void CloseMenu(Enum_Menu menu)
    {
        if (this.is_loading_menu)
        {
            return;
        }

        if (this.shouldShowLoadingScreen(menu))
        {
            this.is_loading_menu = true;

            if (this.menu_history.Count > 0)
            {
                Enum_Menu curr_menu = this.menu_history.Peek();
                this.CloseMenu(curr_menu);
            }

            Menu loading_menu = this.menu_map[Enum_Menu.Loading];
            loading_menu.Open();

            StartCoroutine(CloseMenuCoroutine());
        }
        else
        {
            this.menu_map[menu].Close();
        }


    }

    /**
        <summary>
        Navigates back to the previous menu in the menu history.
        </summary>
    **/
    public void BackMenu()
    {
        if (this.menu_history.Count > 0)
        {
            Enum_Menu curr_menu = this.menu_history.Peek();
            //Debug.Log("Trying to close menu: " + curr_menu);
            this.CloseMenu(curr_menu);

            this.menu_history.Pop();

            Enum_Menu last_menu = this.menu_history.Peek();

            if (last_menu == curr_menu)
            {
                this.menu_history.Pop();
                last_menu = this.menu_history.Peek();
            }

            //Debug.Log("Trying to open menu on BackMenu: " + last_menu);
            this.menu_map[last_menu].Open();
        }
        else
        {
            Debug.LogWarning("Current menu breadcrumb is 0");
        }


    }

    /**
        <summary>
        Determines whether a loading screen should be shown for a specific menu.
        </summary>
        <param name="menu">The Enum_Menu identifier of the menu to be checked.</param>
        <returns>True if a loading screen should be shown, false otherwise.</returns>
    **/
    private bool shouldShowLoadingScreen(Enum_Menu menu)
    {
        if (this.is_need_loading)
        {
            return true;
        }
        else
        {
            return false;
        }

    }

    /**
        <summary>
        Coroutine for loading a menu.
        </summary>
    **/
    private IEnumerator LoadMenuCoroutine()
    {
        Menu loading_menu = this.menu_map[Enum_Menu.Loading];
        loading_menu.Close();

        if (this.menu_history.Count > 0)
        {
            Enum_Menu curr_menu = this.menu_history.Peek();
            this.CloseMenu(curr_menu);
        }

        yield return new WaitForSeconds(1f);

        this.menu_map[this.menu_to_load].Open();
        this.menu_history.Push(menu_to_load);


        this.is_loading_menu = false;
        this.is_need_loading = false;
    }

    /**
        <summary>
        Coroutine for closing a menu.
        </summary>
    **/
    private IEnumerator CloseMenuCoroutine()
    {

        Menu loading_menu = this.menu_map[Enum_Menu.Loading];
        loading_menu.Close();


        yield return new WaitForSeconds(1f);



        this.is_loading_menu = false;
        this.is_need_loading = false;
    }

    /**
        <summary>
        Toggles the flag for displaying the loading screen.
        </summary>
    **/
    public void ToggleLoading()
    {
        this.is_need_loading = !this.is_need_loading;
    }

    /**
        <summary>
        Forces the loading screen to be opened.
        </summary>
    **/
    public void ForceOpenLoading()
    {
        this.menu_map[Enum_Menu.Loading].Open();
    }


    /**
        <summary>
        Forces the loading screen to be closed.
        </summary>
    **/
    public void ForceCloseLoading()
    {
        this.menu_map[Enum_Menu.Loading].Close();
    }

    /**
        <summary>
        Closes all menus.
        </summary>
    **/
    public void CloseAllMenu()
    {
        foreach (Enum_Menu menu in this.menu_map.Keys)
        {
            if (this.menu_map[menu].open)
            {
                this.menu_map[menu].Close();
                this.is_all_closed = false;
            }
        }
    }

    /**
        <summary>
        Checks if all menus are closed.
        </summary>
    **/
    public void IsAllMenuClosed()
    {
        foreach (Enum_Menu menu in this.menu_map.Keys)
        {
            if (!this.menu_map[menu].open)
            {
                this.is_all_closed = true;
            }
        }
    }
}