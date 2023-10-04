using UnityEngine;

namespace Metaversitas.User
{
    public class UserManager: MonoBehaviour
    {
        private bool _isInitialized = false;
        private UserData _userData;
        private UserSession _userSession;

        public void Initialize(UserData userData, UserSession userSession)
        {
            if (_isInitialized)
            {
                Debug.Log("User Manager already initialized!");
                return;
            }

            _isInitialized = true;
            _userData = userData;
            _userSession = userSession;
        }

        public UserData get_userData()
        {
            return _userData;
        }

        public UserSession get_userSession()
        {
            return _userSession;
        }
    }
}