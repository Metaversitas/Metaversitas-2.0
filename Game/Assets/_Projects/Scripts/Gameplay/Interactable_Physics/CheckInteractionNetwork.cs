using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;

public class CheckInteractionNetwork : NetworkBehaviour
{
    [Networked(OnChanged = nameof(OnChangedOnUsing))]
    public NetworkBool IsUsing { get; set; }

    private void Spawned()
    {
        IsUsing = false;
    }
    public static void OnChangedOnUsing(Changed<CheckInteractionNetwork> changed)
    {
        var b = changed.Behaviour;
    }

    public void Toggle()
    {
        IsUsing = !IsUsing;
    }

}
