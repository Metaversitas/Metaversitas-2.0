using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlayerSpawnManager : MonoBehaviour
{
	public static PlayerSpawnManager Instance;

	Spawnpoint[] spawnpoints;

	void Awake()
	{
		Instance = this;
		spawnpoints = GetComponentsInChildren<Spawnpoint>();
	}

    public Vector3 GetSpawnPointAsRandom()
    {
        Transform randomSpawnTransform = spawnpoints[Random.Range(0, spawnpoints.Length)].transform;
        return randomSpawnTransform.position;
    }
}
