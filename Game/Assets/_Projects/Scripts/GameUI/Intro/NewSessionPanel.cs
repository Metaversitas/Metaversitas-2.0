using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace GameUI.Intro
{
	public class NewSessionPanel : MonoBehaviour
	{
		[SerializeField] private InputField _inputName;
		[SerializeField] private InputField _inputPass;
		[SerializeField] private Toggle _allowLateJoin;
		[SerializeField] private TMP_Dropdown _pertemuan;

		private PlayMode _playMode;
        private int _pickedEntryIndex;

        public void Show(PlayMode mode)
		{
			gameObject.SetActive(true);
			_playMode = mode;
			UpdateUI();
		}

		public void Hide()
		{
			gameObject.SetActive(false);
		}

		public void OnEditText()
		{
			UpdateUI();
		}

		private void UpdateUI()
		{
			if(string.IsNullOrWhiteSpace(_inputName.text))
				_inputName.text = "Room1";
		}
		
		public void GetDropdownValue()
		{
            _pickedEntryIndex = _pertemuan.value;

		}

		public void OnCreateSession()
		{
			SessionProps props = new SessionProps();
            switch (_playMode)
            {
                case PlayMode.LabFisika:
                    props.StartMap = MapIndex.LabFisika;
                    break;

                case PlayMode.Candi:
                    // Sesuaikan dengan map yang sesuai
                    props.StartMap = MapIndex.Borobudur;
                    break;

                case PlayMode.Museum:
                    // Sesuaikan dengan map yang sesuai
                    props.StartMap = MapIndex.Museum;
                    break;
                case PlayMode.Malioboro:
                    // Sesuaikan dengan map yang sesuai
                    props.StartMap = MapIndex.Malioboro;
                    break;
            }
            props.PlayMode = _playMode;
			props.RoomName = _inputName.text;
			props.RoomPass = _inputPass.text;
			props.AllowLateJoin = _allowLateJoin.isOn;
			props.RoomPertemuan = _pertemuan.options[_pickedEntryIndex].text;
			
			
			// Pass the session properties to the app - this will unload the current scene and load the staging area if successful
			App.FindInstance().CreateSession(props);
		}
	}
}