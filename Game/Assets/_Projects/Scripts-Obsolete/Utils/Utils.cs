using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public static class Utils
{
    public static Vector3 GetRandomSpawnPoint()
    {
        Vector3 spawnpoint = PlayerSpawnManager.Instance.GetSpawnPointAsRandom();
        return spawnpoint;
    }

    public static void SetRenderLayerInChildren(Transform transform, int layerNumber)
    {
        foreach(Transform trans in transform.GetComponentsInChildren<Transform>(true))
        { trans.gameObject.layer = layerNumber;}
    }
}
