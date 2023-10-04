using System;

namespace Metaversitas.User
{
    [Serializable]
    public enum UserUniversityRole
    {
        Mahasiswa,
        Dosen,
        Administrator,
    }
    
    [Serializable]
    public struct UserData
    {
        private Int64 _facultyID;
        private string _facultyName;
        private string _fullName;
        private string _inGameNickname;
        private string _universityName;
        private string _userID;
        private UserUniversityRole _userUnivRole;
        private Int64 _userUniversityID;

        public UserData(Int64 facultyID, string facultyName, string fullName, string inGameNickname, string universityName, string userID, UserUniversityRole userUnivRole, Int64 userUniversityID)
        {
            _facultyID = facultyID;
            _facultyName = facultyName;
            _fullName = fullName;
            _inGameNickname = inGameNickname;
            _universityName = universityName;
            _userID = userID;
            _userUniversityID = userUniversityID;
            _userUnivRole = userUnivRole;
        }

        public Int64 get_facultyID()
        {
            return _facultyID;
        }

        public string get_facultyName()
        {
            return _facultyName;
        }

        public string get_fullName()
        {
            return _fullName;
        }

        public string get_inGameNickname()
        {
            return _inGameNickname;
        }

        public string get_universityName()
        {
            return _universityName;
        }

        public UserUniversityRole get_userRole()
        {
            return _userUnivRole;
        }

        public string get_userID()
        {
            return _userID;
        }

        public Int64 get_userUniversityID()
        {
            return _userUniversityID;
        }
    }
}