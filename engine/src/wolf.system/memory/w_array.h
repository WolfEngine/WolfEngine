/*
    Project          : Wolf Engine. Copyright(c) Pooya Eimandar (https://PooyaEimandar.github.io) . All rights reserved.
    Source           : Please direct any bug to https://github.com/WolfEngine/Wolf.Engine/issues
    Website          : https://WolfEngine.App
    Name             : w_array.h
    Description      : a dynamic array
    Comment          :
*/

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#include "wolf.h"

    //typedef struct apr_array* w_array;
    typedef struct apr_array_header_t* w_array;

    /*struct  w_header_array
{
    w_mem_pool pool;
    int 	elt_size;
    int 	nelts;
    int 	nalloc;
    char* elts;
};*/

    w_array w_array_init(
        _In_ int pInitSize,
        _In_ int pSizeOfEachElement,
        _In_ w_mem_pool pMemPool);

    /**
     * get an element by index.
     * @param pElementIndex The index of element.
     * @return Location for the an element is avaieble in the array, else it will return NULL
    */
    const void* w_array_get_element(_Inout_ w_array pArray, _In_ int pElementIndex);

    /**
     * Add a new element to an array (as a first-in, last-out stack).
     * @param pArrayIter The array to add an element to.
     * @param pItem an element which is going to add
     * @return Location for the new element in the array.
     * @remark If there are no free spots in the array,
              then this function will, allocate new space for the new element.
    */
    void* w_array_append(_Inout_ w_array pArray, _In_ void* pItem);

    /**
     * Remove an element from an array.
     * @param pArrayIter , The array to remove an element from.
     * @return Location of the element in the array.
     * @remark If there are no elements in the array, NULL is returned.
     */
    void* w_array_remove(_Inout_ w_array pArrayIter);

    /**
     * check table is empty
     * @param pTable , The table to check
     * @return result
     */
    int w_array_is_empty(_In_ w_array pArray);

    /**
     * Remove all elements.
     */
    void w_array_clear(_Inout_ w_array pArray);

#ifdef __cplusplus
}
#endif