using Fusion;
using UnityEngine;
using UnityEngine.UI;
using System;

/// <summary>
/// Visual representation of a Player - the Character is instantiated by the map once it's loaded.
/// This class handles camera tracking and player movement and is destroyed when the map is unloaded.
/// (I.e. the player gets a new avatar in each map)
/// </summary>

public class Character : NetworkBehaviour
{
	[SerializeField] private Text _name;
	[SerializeField] private Animator _animator;
	[SerializeField] private CharacterInteraction _interaction;
    [SerializeField] private PlayerStateManager _playerStateManager;
	private CharacterMovement _characterMovement;

	[SerializeField] private Transform _anchorCharacter;
	[SerializeField] private GameObject _characterCowo;
    [SerializeField] private GameObject _characterCewe;

	public bool cowok;
	public string Role;

    public float moveVelocity = 5f;

	[UnityHeader("Networked Anim Field")]
	[Networked] public Angle yCamRotation { get; set; }
	[Networked] public int xMovement { get; set; }
	[Networked] public int yMovement { get; set; }

	[Networked] public Player Player { get; set; }

	[Networked]
	public bool _isReadInput { get; set; }


    public override void Spawned()
	{
		_isReadInput = false;
        _characterMovement = GetComponent<CharacterMovement>();
        cowok = true;
		if (HasInputAuthority)
		{
			Role = "Dosen";
            // Jika boolean "cowok" adalah true
            
        }

		SpawnCharacterModel();
    }


	private void SpawnCharacterModel()
    {
		if (cowok)
		{
			// Membuat karakter laki-laki sebagai anak dari _anchorCharacter
			var model = Instantiate(_characterCowo, _anchorCharacter.position, _anchorCharacter.rotation);
			model.transform.SetParent(_anchorCharacter);

            // Mengambil animator dari karakter laki-laki dan mengatur ke _animator
            //_animator = model.GetComponent<Animator>();
            _characterMovement.GetAnimator(model);


        }
		else
		{
			// Jika boolean "cowok" adalah false, maka kita mengasumsikan karakter perempuan
			// Membuat karakter perempuan sebagai anak dari _anchorCharacter
			var model = Instantiate(_characterCewe, _anchorCharacter.position, _anchorCharacter.rotation);
			model.transform.SetParent(_anchorCharacter);

            // Mengambil animator dari karakter perempuan dan mengatur ke _animator
            _characterMovement.GetAnimator(model);
        }
	}

	public void SetPlayer(Player player)
    {
		Player = player;
		_interaction.Player = player;
    }

    public void LateUpdate()
	{
		// This is a little brute-force, but it gets the job done.
		// Could use an OnChanged listener on the properties instead.
		// _name.text = Player.Name.Value;
	}
}