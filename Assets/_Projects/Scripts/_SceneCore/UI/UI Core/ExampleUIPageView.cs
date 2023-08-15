using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ExampleUIPageView : UIPageView
{
    protected override void OnOpen()
    {
        base.OnOpen();
        Debug.Log("On Open called");
    }

    protected override void OnClosed()
    {
        base.OnClosed();
        Debug.Log("On Closed called");
    }
}
