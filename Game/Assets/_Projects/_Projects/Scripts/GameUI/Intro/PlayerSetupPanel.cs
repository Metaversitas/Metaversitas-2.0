using UnityEngine;
using UnityEngine.UI;

namespace GameUI.Intro
{
	public class PlayerSetupPanel : MonoBehaviour
	{
		[SerializeField] private Slider _sliderR;
		[SerializeField] private Slider _sliderG;
		[SerializeField] private Slider _sliderB;
		[SerializeField] private Image _color;
		[SerializeField] private GameObject _playerReady;
		[SerializeField] private bool _closeOnReady;

		private App _app;

		public void Show(bool s)
		{
			gameObject.SetActive(s);
		}

		private void OnEnable()
		{
			_app = App.FindInstance();
			_playerReady.SetActive(false);
			_app.AllowInput = false;
		}

		private void OnDisable()
		{
			_app.AllowInput = true;
		}

		public void OnNameChanged(string name)
		{
			Player ply = _app.GetPlayer();
			// Player may be null for a few frames if this happens to be called during a disconnect.
			// In that case the call is irrelevant so we just ignore it.
			if (ply)
			{
				ply.RPC_SetName(name);
			}
		}
	
		public void OnColorUpdated()
		{
			Player ply = _app.GetPlayer();
			if(ply)
			{
				Color c = new Color(_sliderR.value, _sliderG.value, _sliderB.value);
				_color.color = c;
				ply.RPC_SetColor( c);
			}
		}
		
		public void OnToggleIsReady()
		{
			if(_closeOnReady)
				Show(false);
			else
			{
				Player ply = _app.GetPlayer();
				if (ply)
				{
					_playerReady.SetActive(!ply.Ready);
					ply.RPC_SetIsReady(!ply.Ready);
				}
			}
		}
	}
}