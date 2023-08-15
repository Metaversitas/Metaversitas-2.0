using System.Collections;
using UnityEngine;

public class SampleService : SceneService
{
    protected override string GetId() => nameof(SampleService);

    protected override void OnActivate()
    {
        Debug.Log("SampleService is activated");
    }

    protected override void OnTick()
    {
        Debug.Log("OnTick is activated");
    }
}