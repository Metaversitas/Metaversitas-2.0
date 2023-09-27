using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;
public class CharacterMovement : NetworkBehaviour
{
    [SerializeField] private Animator _animator;
    [SerializeField] private Character _character;
    [SerializeField] private PlayerStateManager _gameStateManager;
    public float moveVelocity = 5f;

    [UnityHeader("Networked Anim Field")]
    [Networked] public Angle yCamRotation { get; set; }
    [Networked] public int xMovement { get; set; }
    [Networked] public int yMovement { get; set; }

    public override void Spawned()
    {
        _character = GetComponent<Character>();
    }
    public void GetAnimator(GameObject model)
    {
        _animator = model.GetComponent<Animator>();
    }

    public override void FixedUpdateNetwork()
    {
        if (_character.Player == null) return;

        if (_character.Player.InputEnabled && GetInput(out InputData data))
        {
                _character._isReadInput = true;            

            if (data.GetButton(ButtonFlag.LEFT))
            {
                transform.position -= Runner.DeltaTime * moveVelocity * transform.right;
                xMovement = -1;
            }
            else if (data.GetButton(ButtonFlag.RIGHT))
            {
                transform.position += Runner.DeltaTime * moveVelocity * transform.right;
                xMovement = 1;
            }
            else if (data.GetButton(ButtonFlag.FORWARD))
            {
                transform.position += Runner.DeltaTime * moveVelocity * transform.forward;
                yMovement = 1;
            }
            else if (data.GetButton(ButtonFlag.BACKWARD))
            {
                transform.position -= Runner.DeltaTime * moveVelocity * transform.forward;
                yMovement = -1;
            }
            else // No input
            {
                _character._isReadInput = false;
                xMovement = 0;
                yMovement = 0;
            }

            yCamRotation += data.YCamRotation;
        }

        transform.rotation = Quaternion.Euler(0, (float)yCamRotation, 0);
    }

    public override void Render()
    {
        if (_character._isReadInput)
        {
            _animator.SetFloat("xMovement", xMovement);
            _animator.SetFloat("yMovement", yMovement);
        }
        else
        {
            _animator.SetFloat("xMovement", 0);
            _animator.SetFloat("yMovement", 0);
        }
    }
}
