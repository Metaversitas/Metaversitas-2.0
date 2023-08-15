using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SampleSceneCore : SceneCore
{
    protected override void OnInitialize()
    {
        base.OnInitialize();
        Debug.Log("On Initialize called");
    }
}
